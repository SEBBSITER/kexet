import json
import random
import sys
import time

state = {
    "client_id": None,
    "data_path": None,
    "weights_path": None,
    "initialized": False,
}

def send(msg):
    print(json.dumps(msg), flush=True)

def log(msg):
    print(msg, file=sys.stderr, flush=True)

def handle_init(msg):
    state["client_id"] = msg["client_id"]
    state["data_path"] = msg["data_path"]
    state["weights_path"] = msg["weights_path"]
    state["initialized"] = True

    log (
        f"[worker {state['client_id']}] initialized with data ={state['data_path']} weights={state['weights_path']}"
    )

    send({
        "status": "Ready",
        "client_id": state["client_id"]
    })

def handle_train(msg):
    if not state["initialized"]:
        send({
            "status": "Error",
            "message": "worker not initialized"
        })
        return
    
    round_no = msg["round"]
    epochs = msg["epochs"]
    lr = msg["lr"]

    log (
        f"[worker {state['client_id']}] training round ={round_no} epochs={epochs} lr={lr}"
    )

    # TODO: Do actual training
    loss = 0.5
    accuracy = 0.3
    out_weights = "path"

    send({
        "status": "TrainDone",
        "client_id": state["client_id"],
        "round": round_no,
        "loss": loss,
        "accuracy": accuracy,
        "weights_path": out_weights,
    })

def handle_shutdown(msg):
    if state["client_id"] is None:
        client_id = -1
    else:
        client_id = state["client_id"]

    log(f"worker {client_id} shutting down")

    send({
        "status": "Bye",
        "client_id": client_id
    })

def main():
    for line in sys.stdin:
        line = line.strip()
        if not line:
            continue

        try:
            msg = json.loads(line)
            cmd = msg.get("cmd")

            if cmd == "Init":
                handle_init(msg)
            elif cmd == "Train":
                handle_train(msg)
            elif cmd == "Shutdown":
                handle_shutdown(msg)
                break
            else:
                send({
                    "status": "Error",
                    "message": f"unknown command: {cmd}"
                })
        except Exception as e:
            send({
                "status": "Error",
                "message": str(e),
            })

if __name__ == "__main__":
    main()