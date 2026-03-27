use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};
use std::io::{BufReader, BufRead, Write, stderr};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "cmd")]
enum ClientCommand {
    Init {
        client_id: usize,
        data_path: String,
        weights_path: String,
    },
    Train {
        round: usize,
        epochs: usize,
        lr: f32,
    },
    Shutdown,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "status")]
enum ClientResponse {
    Ready {
        client_id: usize,
    },
    TrainDone {
        client_id: usize,
        round: usize,
        loss: f32,
        accuracy: f32,
        weights_path: String,
    },
    Bye {
        client_id: usize,
    },
    Error {
        message: String,
    }
}

struct Client {
    client_id: usize,
    child: Child,
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
}

impl Client {
    fn spawn(
        client_id: usize,
        python_path: &PathBuf,
        worker_script: &str,
    ) -> Result<Self> {
        let mut child = Command::new(python_path)
            .arg(worker_script)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()?;

        let stdin = child
            .stdin
            .take()
            .ok_or("Failed to capture child stdin")?;

        let stdout = child
            .stdout
            .take()
            .ok_or("Failed to capture child stdout")?;

        Ok(Self {
            client_id,
            child,
            stdin,
            stdout: BufReader::new(stdout),
        })
    }

    fn send(&mut self, cmd: &ClientCommand) -> Result<()> {
        let line = serde_json::to_string(&cmd)?;
        writeln!(self.stdin, "{}", line)?;
        self.stdin.flush()?;
        Ok(())
    }

    fn receive(&mut self) -> Result<ClientResponse>{
        let mut line = String::new();
        let n = self.stdout.read_line(&mut line)?;

        if n == 0 {
            return Err(format!("Worker {} closed stdout", self.client_id).into())
        }

        let response: ClientResponse = serde_json::from_str(line.trim())?;
        Ok(response)
    }

    fn init(&mut self, data_path: String, weights_path: String) -> Result<()> {
        self.send(&ClientCommand::Init {
            client_id: self.client_id,
            data_path,
            weights_path,
        })?;

        match self.receive()? {
            ClientResponse::Ready { client_id } if client_id == self.client_id => Ok(()),
            ClientResponse::Error { message } => Err(format!("Init failed: {message}").into()),
            other => Err(format!("Unexpected init response: {other:?}").into()),
        }
    }

    fn train(&mut self, round: usize, epochs: usize, lr: f32) -> Result<ClientResponse> {
        self.send(&ClientCommand::Train {round, epochs, lr})?;

        match self.receive()? {
            ok @ ClientResponse::TrainDone { .. } => Ok(ok),
            ClientResponse::Error { message } => Err(format!("Train failed: {message}").into()),
            other => Err(format!("Unexpected train response: {other:?}").into()),
        }
    }

    fn shutdown(&mut self) -> Result<()> {
        self.send(&ClientCommand::Shutdown)?;

        match self.receive()? {
            ClientResponse::Bye { client_id } if client_id == self.client_id => Ok(()),
            ClientResponse::Error { message } => Err(format!("Shutdown failed: {message}").into()),
            other => Err(format!("Unexpected shutdown response: {other:?}").into()),
        }
    }

    fn kill(&mut self) -> Result<()> {
        self.child.kill()?;
        Ok(())
    }
}

pub struct ClientPool {
    clients: HashMap<usize, Client>,
}

impl ClientPool {
    pub(crate) fn new() -> Self {
        Self {
            clients: HashMap::new(),
        }
    }

    fn add_worker(&mut self, client: Client) -> Result<()> {
        self.clients.insert(client.client_id, client);
        Ok(())
    }

    pub fn create_clients(
        &mut self, number_clients: usize,
        python_path: &PathBuf,
        worker_script: &str
    ) -> Result<()> {
        for client_id in 0..number_clients {
            let client = Client::spawn(
                client_id,
                python_path,
                worker_script,
            )?;

            self.add_worker(client);
        }
        Ok(())
    }

    pub fn init_all(&mut self) -> Result<()> {
        for client in self.clients.values_mut() {
            client.init("test".parse()?, "test".parse()?)?;
        }
        Ok(())
    }

    pub fn train_all(
        &mut self,
        rounds: usize,
        epochs: usize,
        lr: f32,
    ) -> Result<()> {
        for client in self.clients.values_mut() {
            let result = client.train(rounds, epochs, lr)?;
            if let ClientResponse::TrainDone {
                client_id,
                round,
                loss,
                accuracy,
                weights_path,
            } = result {
                println!("Training done for client {} -> acc {:.4}",
                client_id, accuracy);
            }
        }
        Ok(())
    }

    pub fn shutdown_all(&mut self) -> Result<()> {
        for client in self.clients.values_mut() {
            client.shutdown()?;
        }
        Ok(())
    }
}

