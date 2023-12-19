import init, {show_result_html, solve} from './pkg/secret_santa_wasm.js';

async function run() {
    await init();

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
        const inputData = inputElement.value;

        try {
            const solution = solve(inputData);
            outputElement.innerHTML = show_result_html(solution);
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

}

run();
