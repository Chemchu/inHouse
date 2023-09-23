import html from "@elysiajs/html";
import { Elysia } from "elysia";
import * as elements from "typed-html";
import Dashboard from "./application/services/dashboard/Dashboard";
import controller from "./application/controller";

const app = new Elysia()
  .use(html())
  .use(controller())
  .get("/clicked", () => <div>Clicked!</div>)
  .get("/dashboard", () => <Dashboard />)
  .post("/click-me", () => <div class="hover:text-blue-500">Clicked!</div>)
  .listen(3000);

console.log(`🦊 Elysia is running at ${app.server?.hostname}:${app.server?.port}`);
