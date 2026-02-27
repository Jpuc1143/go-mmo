//import { createRoot } from "react-dom/client";

//import App from "./App.tsx";
import GameRenderer from "./View/GameRenderer.ts";
import GameGateway from "./Gateway/GameGateway";
import { Game } from "./Domain/Game.ts";

const gateway = new GameGateway(new URL("ws://localhost:3000/ws"));
const game = new Game(gateway);
const renderer = new GameRenderer(game);

(async () => renderer.start())();

//createRoot(document.getElementById("pixi-container")!).render(<App />);
