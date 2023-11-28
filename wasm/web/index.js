import * as wasm from "secret-santa-wasm";

wasm.init();

const solveButton = document.getElementById("solve");

var inputElement = document.getElementById("inputData");

solveButton.addEventListener("click", event => {
    // Get the value (text) from the input element
    var inputData = inputElement.value;

    var outputValue = solve(inputData);

    // Display the output in an HTML element
    var outputElement = document.getElementById("output");
    outputElement.innerHTML = outputValue;
});

function solve(inputData) {
    var res = wasm.solve(inputData);
    // json to js
    var map = JSON.parse(res);
    var out = "";
    for (let key in map) {
        if (map.hasOwnProperty(key)) {
            console.log(key, map[key]);
            out += `<ul><a href="${map[key]}" target="_blank">${key}</a></ul>`;
        }
    }
    return out;
}

var exampleYaml = `participants:
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
