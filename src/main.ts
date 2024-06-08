import { bootstrapApplication } from "@angular/platform-browser";
import { appConfig } from "./app/app.config";
import { AppComponent } from "./app/app.component";
import {listen} from "@tauri-apps/api/event";

interface NoteWrapper {
    velocity: number,
    note: string
}

const list = listen("MIDI_NOTE", (event) => {
    let payload = event.payload as NoteWrapper;
    console.log("RECEBIDO: " + payload.note + " " + payload.velocity)
    console.log(event.payload)
});

bootstrapApplication(AppComponent, appConfig).catch((err) =>
  console.error(err),
);
