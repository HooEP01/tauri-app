import { HttpClient } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { Observable, catchError, of, switchMap } from 'rxjs';

@Injectable({
    providedIn: 'root'
})
export class DashboardService {

    privaryPolicy: any;

    constructor(
        private http: HttpClient,
    ) { }

    fetchPrivatePolicy(): Observable<boolean> {
        return this.http.get('https://jsonplaceholder.typicode.com/todos/1').pipe(
            switchMap((res) => {
                this.privaryPolicy = res;
                return of(true)
            }),
            catchError(() => of(false)),
        );
    }
}