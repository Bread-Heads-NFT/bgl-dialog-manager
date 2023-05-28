const path = require("path");

const programDir = path.join(__dirname, "..", "programs");
function getProgram(dir, programName) {
  return path.join(programDir, dir, "target", "deploy", programName);
}

module.exports = {
  validator: {
    commitment: "processed",
    programs: [
      {
        label: "Bgl Dialog Manager",
        programId: "D1ALGLQpQ1mYs2QpNfuEVposQp5fwvJAep3y2gUgLo6",
        deployPath: getProgram("bgl-dialog-manager", "bgl_dialog_manager.so"),
      },
    ],
  },
};
