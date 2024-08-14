export enum RustFunctionName {
    // dataSource service
    /**
     * Conecta ao dispositivo midi
     */
    connectMidi = "start_listening_midi",
    /**
     * Desconecta do dispositivo midi previamente conectado.
     * Faz nada se nao tiver conectado
     */
    stopMidi = "disconnect_midi",
    listMidiDevices = "list_midi_devices",
    /**
     * (musicId: String)
     * 
     * Comeca a enviar o evento midiReadNote
     */
    startGame = "start_game",
    /**
     * Devolve a lista de todas as musicas disponiveis
     */
    listMusics = "list_musics",
}

export enum RustEventsName {
    midiNote = "MIDI_INPUT_NOTE",
    midiReadNote = "MIDI_READ_NOTE"
}