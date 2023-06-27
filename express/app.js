var express = require("express");
var path = require("path");
var cookieParser = require("cookie-parser");
var logger = require("morgan");
const { readFile } = require('node:fs/promises');
const { join } = require('path');

var app = express();

app.use(logger("dev"));
app.use(express.json());
app.use(express.urlencoded({ extended: false }));
app.use(cookieParser());
app.use(express.static(path.join(__dirname, "public")));

app.use("/test", async (req, res, next) => {
  const contents = await readFile(join(__dirname, 'example_849K.json'), { encoding: 'utf8' });
  res.send(contents);
});


app.use((err, req, res, next) => {
  if (err instanceof Error) {
    res.status(err.errorCode);
    res.json(err.message);
  }
})

module.exports = app;
