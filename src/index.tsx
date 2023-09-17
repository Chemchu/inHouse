import html from "@elysiajs/html";
import { Elysia } from "elysia";
import * as elements from "typed-html";

const app = new Elysia()
  .use(html())
  .get("/", ({ html }) =>
    html(
      <BaseHtml>
        <body class="flex w-full h-screen justify-center items-center" hx-post="/click-me" hx-swap="outerHTML">
          <button>Click me!</button>
        </body>
      </BaseHtml>
    )
  )
  .post("/click-me", () => <div class="hover:text-blue-500">Clicked!</div>)
  .listen(3000);

console.log(`🦊 Elysia is running at ${app.server?.hostname}:${app.server?.port}`);

const host = Bun.env.SUPABASE_URL;
const password = Bun.env.SUPABASE_PASSWORD;
const anonKey = Bun.env.SUPABASE_ANON_KEY;

console.log(host, password, anonKey);

const BaseHtml = ({ children }: elements.Children) => `
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8" />
  <meta http-equiv="X-UA-Compatible" content="IE=edge" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <script src="https://unpkg.com/htmx.org@1.9.5"></script>
  <script src="https://cdn.tailwindcss.com?plugins=forms,typography,aspect-ratio,line-clamp"></script>
  <title>The BETH stack</title>
</head>
<body>
  ${children}
</body>
</html>
`;
