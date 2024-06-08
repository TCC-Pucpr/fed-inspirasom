import { Routes } from "@angular/router";
import { PartituraComponent } from "./core/screens/partitura/partitura.component";

export const routes: Routes = [
    { path: '**', component: PartituraComponent }
];
