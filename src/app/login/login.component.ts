import { Component } from "@angular/core";
import { UntypedFormBuilder, UntypedFormGroup } from "@angular/forms";
import { Router } from "@angular/router";
import { invoke } from "@tauri-apps/api/tauri";
import { getClient, Body } from '@tauri-apps/api/http';
import { emit, listen } from '@tauri-apps/api/event'
// const { listen } = window.__TAURI__.event;
import { WebviewWindow } from '@tauri-apps/api/window'

@Component({
    selector: 'login',
    templateUrl: './login.component.html',
})

export class LoginComponent {


    greetingMessage = "";

    itemForm!: UntypedFormGroup;


    constructor(
        private formBuilder: UntypedFormBuilder,
        private router: Router,
    ) { }

    async ngOnInit() {

        document.addEventListener('DOMContentLoaded', () => {
            invoke('close_splashscreen')
        })


        const webview = new WebviewWindow('local-unique', {
            url: 'https://github.com/tauri-apps/tauri',
        })


        webview.once('tauri://created', function () {
            console.log('created');
        })

        webview.once('tauri://error', function (e) {
            console.log('error', e);
        })

        const mainWindow = WebviewWindow.getByLabel('main');
        console.log('mainWindow', mainWindow);

        this.itemForm = this.formBuilder.group({
            email: "",
            password: "",
        });


        const unlistening = listen("message-back-end", (event) => {
            const { message } = event.payload as any;
            console.log('unlistening', message);
        })

        const unlisten = listen("send-message", (event) => {
            const { message } = event.payload as any;
            console.log(message);
            // const { color } = event.payload;
            // document.body.style.backgroundColor = color;
        });

    }

    message() {
        console.log('message')
        emit('send-message', {
            message: 'Tauri is awesome!'
        })
    }

    async submit() {
        const client = await getClient();
        const response = await client.post('https://jsonplaceholder.typicode.com/posts',
            Body.json({
                title: 'foo',
                body: 'bar',
                userId: 1,
            }),
        );
        console.log(response);
        this.router.navigateByUrl('dashboard');
    }

    greet(event: SubmitEvent, name: string): void {
        event.preventDefault();

        // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
        invoke<string>("greet", { name }).then((text) => {
            this.greetingMessage = text;
        });
    }


}
