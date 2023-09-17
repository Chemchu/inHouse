import * as elements from "typed-html";
import BaseHtml from "../../../component/BaseHtml";

const Home = () => {
  return (
    <BaseHtml>
      <div>
        <h1>Home</h1>

        <button hx-get="/clicked" hx-swap="outerHTML">
          Click me!
        </button>
      </div>
    </BaseHtml>
  );
};

export default Home;
