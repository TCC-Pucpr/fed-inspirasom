import { Routes, provideRouter } from "@angular/router";
import { PartituraComponent } from "./core/screens/partitura/partitura.component";
import { ApplicationConfig } from "@angular/core";

export const routes: Routes = [
    
    { path: '**', component: PartituraComponent }
];

export const routesConfig: ApplicationConfig = {
    providers: [provideRouter(routes)],
};
  