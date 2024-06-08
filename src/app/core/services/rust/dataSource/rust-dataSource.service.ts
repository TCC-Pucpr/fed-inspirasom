import { Injectable } from '@angular/core';
import { invoke } from '@tauri-apps/api/tauri';
import { RustFunctionName } from '../rust-functions.enum';

@Injectable({
  providedIn: 'root'
})
export class RustDataSourceService {

  constructor() { }

   /**
    * Função exemplo de comunicação entre rust e angular
    * @param event Evento teste
    * @param message mensagem que irá para o rust
    * @returns string com uma mensagem montada
    */
   public async greet(event: SubmitEvent, message: string): Promise<string> {
    event.preventDefault();
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    return await invoke<string>(RustFunctionName.greet, { name: message } );
  }

}
