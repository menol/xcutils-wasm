import React from 'react';
import { render } from 'react-dom';
import App from './App.jsx';
import './App.css';
import init, { proto_to_typescript, proto_to_swift,proto_to_kotlin } from './wasm/xcutils_wasm.js';

async function loadWasm() {
  await init();
}

loadWasm().then(() => {
  render(<App />, document.getElementById('app'));
});