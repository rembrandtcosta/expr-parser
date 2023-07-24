// Import our custom CSS
import '../scss/styles.scss';

import init, { sy } from "../../infix-parser/pkg/infix_parser.js";

// Import only the Bootstrap components we need
import { Popover } from 'bootstrap';

// Create an example popover
document.querySelectorAll('[data-bs-toggle="popover"]')
  .forEach(popover => {
    new Popover(popover)
  })

const form = document.querySelector("form");

form.addEventListener("submit", (e) => {
  e.preventDefault();
  
  const expression = document.getElementById("expression").value;

  const result = document.getElementById("result");

  result.value = sy(expression);
});

init()
