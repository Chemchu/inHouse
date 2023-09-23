import Elysia from "elysia";
import Home from "../../pages/Home";

const homeService = () => {
    return new Elysia()
        .get("/", Home)
}

export default homeService;