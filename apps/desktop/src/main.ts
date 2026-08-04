import { createApp } from "vue";
import { createPinia } from "pinia";
import "@what-command/halo/index.css";
import App from "./App.vue";
import { router } from "./router";

const app = createApp(App);

app.use(createPinia()).use(router);

app.config.errorHandler = (error, _instance, info) => {
  console.error("Global error:", error, info);
};

app.mount("#app");
