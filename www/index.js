import init from "snake_game";

init().then((wasm) => {
  wasm.greet("Harrison");
  console.log("okay!");
});
