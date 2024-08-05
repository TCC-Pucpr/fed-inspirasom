import { Routes, provideRouter } from "@angular/router";
import { ApplicationConfig } from "@angular/core";
import { GamificadaComponent } from "./core/screens/gamificada/gamificada.component";
import { MenuDashboardsComponent } from "./core/screens/menu-dashboards/menu-dashboards.component";

export const routes: Routes = [
    { path: 'dashboards', component: MenuDashboardsComponent },
    { path: 'gamificada', component: GamificadaComponent },
    { path: '**', redirectTo: 'dashboards' }
];

export const routesConfig: ApplicationConfig = {
    providers: [provideRouter(routes)],
};
  