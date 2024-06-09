import { Routes, provideRouter } from "@angular/router";
import { ApplicationConfig } from "@angular/core";
import { PartituraComponent } from "./core/screens/menu-gamificado/telas/partitura/partitura.component";
import { MenuGamificadoComponent } from "./core/screens/menu-gamificado/menu-gamificado.component";

export const routes: Routes = [

    { path: 'partitura', component: PartituraComponent },
    { path: 'menu-gamificado', component: MenuGamificadoComponent },
    { path: '**', component: MenuGamificadoComponent }
];

export const routesConfig: ApplicationConfig = {
    providers: [provideRouter(routes)],
};
  