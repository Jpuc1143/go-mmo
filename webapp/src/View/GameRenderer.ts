import { Application, Assets, Graphics, Sprite } from "pixi.js";
import { Viewport } from "pixi-viewport";
import { Coord } from "../Domain/Coord";
import { Color } from "../Domain/Color";
import { Game } from "../Domain/Game";

const intersectionSize = 100;
const stoneRadius = 40;

export default class GameRenderer {
  readonly game: Game;

  readonly app = new Application();
  viewport!: Viewport;
  readonly stones = new Map<string, Graphics>();

  public constructor(game: Game) {
    this.game = game;
  }

  public async start() {
    // Initialize the application
    await this.app.init({ background: "#1099bb", resizeTo: window });
    this.viewport = new Viewport({
      events: this.app.renderer.events,
    });
    this.app.stage.addChild(this.viewport);

    this.viewport.drag().decelerate();
    this.viewport.moveCenter(0, 0);

    // Append the application canvas to the document body
    document.getElementById("pixi-container")!.appendChild(this.app.canvas);
    // Load the bunny texture
    const texture = await Assets.load("/assets/bunny.png");

    // Create a bunny Sprite
    const bunny = new Sprite(texture);

    // Center the sprite's anchor point
    bunny.anchor.set(0.5);

    // Move the sprite to the center of the screen
    bunny.position.set(0, 0);

    // Add the bunny to the stage
    this.viewport.addChild(bunny);

    // Listen for animate update
    this.app.ticker.add((time) => {
      bunny.rotation += 0.1 * time.deltaTime;
    });

    await this.game.start(this);

    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    this.viewport.addEventListener("clicked", (e: any) => {
      const x = Math.round(e.world.x / intersectionSize);
      const y = Math.round(e.world.y / intersectionSize);
      const coord = new Coord(x, y);

      // TODO only place if mouse is inside stone area.
      this.game.placeStone(coord);
    });

    this.viewport.on("drag-end", () => {
      //TODO
      //const x = Math.round(e.world.x / intersectionSize);
      //const y = Math.round(e.world.y / intersectionSize);
      //game.moveView(new Coord(x, y));
    });
  }

  public placeStone(coord: Coord, color: Color) {
    if (this.stones.has(coord.hash())) {
      this.stones.get(coord.hash())!.destroy();
    }

    const graphic = new Graphics().circle(0, 0, stoneRadius);

    switch (color) {
      case Color.Black:
        graphic.fill(0x000000);
        break;
      case Color.White:
        graphic.fill(0xffffff);
        break;
    }

    graphic.x = coord.x * intersectionSize;
    graphic.y = coord.y * intersectionSize;

    this.stones.set(coord.hash(), graphic);
    this.viewport.addChild(graphic);
  }

  public removeStone(coord: Coord) {
    if (this.stones.has(coord.hash())) {
      this.stones.get(coord.hash())!.destroy();
    }
  }
}
