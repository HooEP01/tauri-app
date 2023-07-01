import { Routes } from '@angular/router';
import { LoginComponent } from './login/login.component';
import { DashboardComponenet } from './dashboard/dashboard.component';

export const routes: Routes = [
    { path: '', component: LoginComponent },
    { path: 'dashboard', component: DashboardComponenet },
];