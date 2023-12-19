import * as wasm from "secret-santa-wasm";

wasm.init();

const solveButton = document.getElementById("solve");
const clearInputButton = document.getElementById("clearInput");

const inputElement = document.getElementById("inputData");
const outputElement = document.getElementById("output");
const solveErrorsElement = document.getElementById("solveErrors");

clearInputButton.addEventListener("click", event => {
    inputElement.value = "";
});

solveButton.addEventListener("click", event => {
    // Clear results and errors displayed
    outputElement.innerHTML = "";
    solveErrorsElement.innerText = "";

    // Get the value (text) from the input element
    const inputData = inputElement.value;

    try {
        var assignmentsLinks = wasm.solve(inputData);
        outputElement.innerHTML = wasm.show_result_html(assignmentsLinks);
    } catch (error) {
        solveErrorsElement.innerText = error;
        console.error(error);
    }
});

const exampleYaml = `participants:
  - Alice
  - Bob
  - Carol
  - David
already_gifted_before:
  Alice:
    - Carol
  Carol:
    - Bob
couples:
  - - Alice
    - Bob
  - - Carol
    - David`;

document.getElementById("loadExample")
    .addEventListener("click", function () {
        inputElement.value = exampleYaml;
    });
