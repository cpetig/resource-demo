import { getDirectories as getDirectories2} from "./filesystem.js";

let _env;
export function _setEnv (envObj) {
  _env = Object.entries(envObj);
}

export const environment = {
  getEnvironment () {
    if (!_env) _env = [];
    return _env;
  }
};

export class TerminalOutput {
  constructor(a) { this.value = a; }
}
export class TerminalInput {
  constructor(a) { this.value = a; }
}

class ComponentExit extends Error {
  constructor(code) {
    super(`Component exited ${code === 0 ? 'successfully' : 'with error'}`);
    this.code = code;
  }
}

export const exit = {
  exit (status) {
    throw new ComponentExit(status.tag === 'err' ? 1 : 0);
  }
};

export const stdin = {
  getStdin () {
    return 0;
  }
};

export const stdout = {
  getStdout () {
    return 1;
  }
};

export const stderr = {
  getStderr () {
    return 2;
  }
};

export const terminalInput = {
  dropTerminalInput () {

  }
};

export const terminalOutput = {
  dropTerminalOutput () {

  }
};

export const terminalStderr = {
  getTerminalStderr () {
    return new TerminalInput(0);
  }
};

export const terminalStdin = {
  getTerminalStdin () {
    return new TerminalOutput(1);
  }
};

export const terminalStdout = {
  getTerminalStdout () {
    return new TerminalOutput(2);
  }
};

export const { getEnvironment } = environment;
export const { getStderr } = stderr;
export const { getStdin } = stdin;
export const { getStdout } = stdout;
export const { dropTerminalInput } = terminalInput;
export const { dropTerminalOutput } = terminalOutput;
export const { getTerminalStderr } = terminalStderr;
export const { getTerminalStdin } = terminalStdin;
export const { getTerminalStdout } = terminalStdout;

export const { getDirectories } = getDirectories2;

