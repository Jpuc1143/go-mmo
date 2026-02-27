//import { createRoot } from "react-dom/client";

//import App from "./App.tsx";
import GameRenderer from "./View/GameRenderer.ts";
import GameGateway from "./Gateway/GameGateway";
import { Game } from "./Domain/Game.ts";

const url = new URL(
  "ws",
  import.meta.env.BACKEND_URL || "http://localhost:3000",
);
const gateway = new GameGateway(url);
const game = new Game(gateway);
const renderer = new GameRenderer(game);

(async () => renderer.start())();

//createRoot(document.getElementById("pixi-container")!).render(<App />);
