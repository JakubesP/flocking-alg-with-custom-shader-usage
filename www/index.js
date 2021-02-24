const canvas = document.getElementById("canvas");
const gl = canvas.getContext("webgl2", { antialias: true });

if (!gl) {
  alert("Failed to initialize WebGL");
}

const fullscreenMode = canvas.getAttribute("fullscreen-mode") != null;

// only for non-fullscreen view
const setInitialCanvasParams = () => {
  if (!fullscreenMode) {
    canvas.width = window.innerWidth;
    canvas.clientWidth = window.innerWidth;
    canvas.height = window.innerHeight;
    canvas.clientHeight = window.innerHeight;
    gl.viewport(0, 0, canvas.width, canvas.height);
  }
};

// only for fullscreen view
const updateCanvasParams = () => {
  canvas.width = window.innerWidth;
  canvas.clientWidth = window.innerWidth;
  canvas.style.width = window.innerWidth;

  canvas.height = window.innerHeight;
  canvas.clientHeight = window.innerHeight;
  canvas.style.height = window.innerHeight;

  gl.viewport(0, 0, canvas.width, canvas.height);
};

// game
(async () => {
  try {
    const engine = await import("wasm-app");

    if (!fullscreenMode) setInitialCanvasParams();
    else updateCanvasParams();

    // Project namespace
    const MyProject = {};
    MyProject.lastTick = performance.now();
    MyProject.lastRender = MyProject.lastTick;
    MyProject.tickLength = 1.0;

    const appState = await new engine.AppState(
      MyProject.lastTick,
      canvas.clientWidth,
      canvas.clientHeight
    );

    const render = (tFrame) => {
      // you can use stopMain as ID to stop project loop
      MyProject.stopMain = window.requestAnimationFrame(render);
      let nextTick = MyProject.lastTick + MyProject.tickLength;
      let numTicks = 0;

      if (tFrame > nextTick) {
        let timeSinceTick = tFrame - MyProject.lastTick;
        numTicks = Math.floor(timeSinceTick / MyProject.tickLength);
      }

      // update
      queueUpdates(numTicks);

      // render
      appState.render(); // tFrame

      MyProject.lastRender = tFrame;
    };

    let lastUpdate = Date.now();

    const queueUpdates = (numTicks) => {
      for (let i = 0; i < numTicks; ++i) {
        MyProject.lastTick = MyProject.lastTick + MyProject.tickLength;

        if (
          fullscreenMode &&
          (window.innerHeight != canvas.height ||
            window.innerWidth != canvas.width)
        ) {
          updateCanvasParams();
        }

        appState.update(
          MyProject.lastTick,
          canvas.clientWidth,
          canvas.clientHeight
        );
      }
    };

    render();
  } catch (err) {
    console.log(err);
  }
})();
