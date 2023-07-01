import { NgModule } from "@angular/core";
import { BrowserModule } from "@angular/platform-browser";
import { AppComponent } from "./app.component";
import { MatFormFieldModule } from '@angular/material/form-field';
import { FormsModule, ReactiveFormsModule } from "@angular/forms";
import { HttpClientModule } from '@angular/common/http';
import { MatInputModule } from '@angular/material/input'; 
import { RouterModule } from '@angular/router';
import { routes } from "./app.routes";
import { LoginComponent } from "./login/login.component";
import { DashboardComponenet } from "./dashboard/dashboard.component";
import { MatButtonModule } from '@angular/material/button'; 
import { DashboardService } from "./dashboard/dashboard.service";

@NgModule({
  declarations: [AppComponent, LoginComponent, DashboardComponenet],
  imports: [
    RouterModule.forRoot(routes),
    BrowserModule,
    MatFormFieldModule,
    ReactiveFormsModule,
    FormsModule,
    HttpClientModule,
    MatButtonModule,
    MatInputModule,
  ],
  providers: [DashboardService],
  bootstrap: [AppComponent],
})
export class AppModule {}
