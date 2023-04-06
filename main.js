// import './style.css'
// import javascriptLogo from './javascript.svg'
// import viteLogo from '/vite.svg'
// import { setupCounter } from './counter.js'
//
// document.querySelector('#app').innerHTML = `
//   <div>
//     <a href="https://vitejs.dev" target="_blank">
//       <img src="${viteLogo}" class="logo" alt="Vite logo" />
//     </a>
//     <a href="https://developer.mozilla.org/en-US/docs/Web/JavaScript" target="_blank">
//       <img src="${javascriptLogo}" class="logo vanilla" alt="JavaScript logo" />
//     </a>
//     <h1>Hello Vite!</h1>
//     <div class="card">
//       <button id="counter" type="button"></button>
//     </div>
//     <p class="read-the-docs">
//       Click on the Vite logo to learn more
//     </p>
//   </div>
// `
//
// setupCounter(document.querySelector('#counter'))

import { invoke } from '@tauri-apps/api/tauri'
import {emit } from '@tauri-apps/api/event'

let greetInputEl;
let greetMsgEl;

async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
}

window.addEventListener("DOMContentLoaded", () => {
    console.log("hello")
    emit('load', event)
});

window.addEventListener("load", (event) => {
    console.log("hello")
    emit('load', event)
    let button = document.createElement('button');
    button.textContent = "click me i'm a test";
    button.addEventListener('click', () => {
        emit('test-click', {
            theMessage: 'Tauri is awesome!',
        })
    });
    document.getElementsByClassName('a-button-stack')[0].appendChild(button);
});




