import Elysia from "elysia";
import authService from "./services/authService";
import homeService from "./services/homeService";
import Html from "@kitajs/html"

const controller = () => {
  return new Elysia()
    .use(authService())
    .use(homeService());
};

export default controller;
