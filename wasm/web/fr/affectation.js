import init, {deobfuscate_name} from '../pkg/secret_santa_wasm.js';

async function run() {
    await init();

    const queryString = window.location.search;
    const urlParams = new URLSearchParams(queryString);
    console.log(`urlParams:${urlParams}`);

    const giver = urlParams.get('g');
    const recipient_obf = urlParams.get('r');
    const seed = urlParams.get('s');

    const recipient = deobfuscate_name(recipient_obf, seed);

    let resultParagraph = document.createElement('p');
    resultParagraph.textContent = `${giver}, tu es le Secret Santa de ${recipient}`;

    document.body.appendChild(resultParagraph);

}

run();
