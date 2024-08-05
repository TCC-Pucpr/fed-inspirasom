import { bootstrapApplication } from "@angular/platform-browser";
import { AppComponent } from "./app/app.component";
import { routesConfig } from "./app/app.routes";

bootstrapApplication(AppComponent, routesConfig).catch((err) =>
  console.error(err),
);
