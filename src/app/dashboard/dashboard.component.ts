import { Component, OnInit } from "@angular/core";
import { Router } from "@angular/router";
import { DashboardService } from "./dashboard.service";
import { tap } from "rxjs";
import { invoke } from '@tauri-apps/api'

@Component({
    selector: 'dashboard',
    templateUrl: './dashboard.component.html',
})

export class DashboardComponenet implements OnInit {

    constructor(
        private service: DashboardService,
        private router: Router,
    ) {}

    ngOnInit() { 
        invoke('background_function_1').then((res) => {
            return console.log(res);
        });
        this.service.fetchPrivatePolicy().pipe(
            tap((res) => {
                if (res) {
                    console.log(this.service.privaryPolicy);
                }
            })
        ).subscribe();
    }

    submit() {
        this.router.navigateByUrl('');
    }
}