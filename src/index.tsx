import { Elysia, t } from "elysia";
import controller from "./application/controller";
import { html } from "@elysiajs/html";
import "@kitajs/html/register";

const app = new Elysia()
  .use(html())
  .use(controller())
  .get("/clicked", () => <div>Clicked!</div>)
  .listen(3000);

console.log(`🦊 Elysia is running at ${app.server?.hostname}:${app.server?.port}`);
