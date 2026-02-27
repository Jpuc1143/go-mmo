import { Game } from "../Domain/Game";
import { GameClientCommand } from "./GameClientCommand";
import { GameServerMessage } from "./GameServerMessage";

export default class GameGateway {
  readonly url: URL;
  socket!: WebSocket;

  constructor(url: URL) {
    this.url = url;
  }

  public start(game: Game) {
    this.socket = new WebSocket(this.url);

    this.socket.onerror = () => console.log("Server connection error");
    this.socket.onopen = () => {
      console.log("Server connection open");
    };

    this.socket.onmessage = (e) => {
      const msg = JSON.parse(e.data) as GameServerMessage;
      console.log(msg);
      game.receiveServerMessage(msg);
    };
  }

  public sendCommand(command: GameClientCommand) {
    console.log("Sending command", command);
    const cmd = JSON.stringify(command);
    this.socket.send(cmd);
  }
}
