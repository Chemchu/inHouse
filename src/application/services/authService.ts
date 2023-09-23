import Elysia from "elysia";
import SignIn from "../../pages/SignIn";
import SignUp from "../../pages/SignUp";

const authService = () => {
    return new Elysia()
        .get("/sign-in", SignIn)
        .get("/sign-up", SignUp)
}

export default authService;