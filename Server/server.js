
const { default: axios } = require('axios');

const apiKey = process.env.OPENAI_API_KEY;
const axios_openai = axios.create({
    headers: { Authorization: "Bearer " + apiKey }
});

const bodyParser = require('body-parser');

const express = require('express');
const app = express();
const port = process.env.PORT || 3000;

app.get("/", (req, res) => {
    res.send('connection established');
});

const parser = bodyParser.json();

app.post("/", parser, (req, res) => {
    let content, ul, instruction = "";
    try {
        content = req.body.original_message;
        instruction =  req.body.instruction;
        ul = req.body.user_language;
    } catch (error) {
        console.log(error);
        res.send('error');
        return;
    }
        axios_openai.post('https://api.openai.com/v1/chat/completions', {
        "model": "gpt-3.5-turbo",
        "messages": [
            {"role": "system", "content": `You are a helpful assistant. Answer using ${ul}`},
            {"role": "user", "content": `${instruction} this: ${content}`}
        ],
        "temperature": .4
    }).then((result) => {
        res.send(result.data.choices[0].message.content);
    }).catch((err) => {
        res.send(err.message);
    })

});

app.listen(port);

