import { Routes, provideRouter } from "@angular/router";
import { ApplicationConfig } from "@angular/core";
import { GamificadaComponent } from "./core/screens/gamificada/gamificada.component";
import { MenuDashboardsComponent } from "./core/screens/menu-dashboards/menu-dashboards.component";
import { provideAnimations } from "@angular/platform-browser/animations";
import { MenuGamificadaComponent } from "./core/screens/menu-gamificada/menu-gamificada.component";

export const routes: Routes = [
    { path: 'dashboards', component: MenuDashboardsComponent },
    { path: 'gamificada', component: GamificadaComponent },
    { path: 'menu-gamificada', component: MenuGamificadaComponent },
    { path: '**', redirectTo: 'dashboards' }
];

export const routesConfig: ApplicationConfig = {
    providers: [provideRouter(routes), provideAnimations()],
};
  