import html from "@elysiajs/html";
import { Elysia } from "elysia";
import * as elements from "typed-html";
import Home from "./application/services/home/Home";
import SignIn from "./application/services/auth/signin/SignIn";
import Dashboard from "./application/services/dashboard/Dashboard";

const app = new Elysia()
  .use(html())
  .get("/clicked", () => <div>Clicked!</div>)
  .get("/", () => <Home />)
  .get("/sign-in", () => <SignIn />)
  .get("/dashboard", () => <Dashboard />)
  .post("/click-me", () => <div class="hover:text-blue-500">Clicked!</div>)
  .listen(3000);

console.log(`🦊 Elysia is running at ${app.server?.hostname}:${app.server?.port}`);

// const host = Bun.env.SUPABASE_URL;
// const password = Bun.env.SUPABASE_PASSWORD;
// const anonKey = Bun.env.SUPABASE_ANON_KEY;

// console.log(host, password, anonKey);
