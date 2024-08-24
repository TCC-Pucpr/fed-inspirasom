/**
 * Todas as funcoes do rust
 * 
 * chamadas via `invoke`
 * 
 * Todas essas são thread safe
 */
export enum RustFunctionName {
    // dataSource service
    /**
     * Conecta a um dispositivo midi especifico
     * 
     * O unico parametro desta função é o nome do dispositivo, que é obtido
     * na função `listMidiDevices`
     *
     * Retorna erro caso ja tiver conectado a algum dispositivo.
     */
    connectMidi = "start_listening_midi",
    /**
     * Tenta conectar a um dispositivo midi padrão
     * 
     * Esse dispositivo padrão é o nosso próprio, que vamos 
     * saber qual que é o nome.
     * 
     * Retorna erro se não encontrou.
     */
    quickConnectMidi = "quick_midi_connect",
    /**
     * Desconecta do dispositivo atualmente conectado
     * 
     * Retorna `true` se desconectou, ou `false` se nao tem nenhum dispositivo conectado
     */
    disconnectMidi = "disconnect_midi",
    /**
     * Desconecta do dispositivo midi previamente conectado.
     * Faz nada se nao tiver conectado
     */
    stopMidi = "disconnect_midi",
    /**
     * Devolve uma lista de string com o nome da porta de todos
     * os dispositivos conectados
     */
    listMidiDevices = "list_midi_devices",
    /**
     * (musicId: String)
     * 
     * Comeca a enviar o evento midiReadNote
     */
    startGame = "start_game",
    /**
     * Notifica o lado do rust para parar de enviar eventos de notas temporariamente
     */
    pauseGame = "pause_game",
    /**
     * Notifica o lado do rust para encerrar a emicao de eventos de notas
     */
    stopgame = "stop_game",
    /**
     * Notifica o lado do rust para voltar a emitir eventos de notas
     */
    resumeGame = "resume_game",
    /**
     * Devolve a lista de todas as musicas disponiveis (`MidiMusicList`)
     */
    listMusics = "list_musics",
}

/**
 * eventos que enviam vários sinais para o front
 * 
 * chamados via `listen`
 */
export enum RustEventsName {
    /**
     * Evento que periodicamente envia `MidiSignal` que vem do arduino.
     * 
     * Chamar `RustFunctionName.connectMidi` para começar a emitir.
     */
    midiNote = "MIDI_INPUT_NOTE",
    /**
     * Evento que periodicamente envia `MidiSignal`, que vem do arquivo midi 
     * atualmente sendo tocado. 
     * 
     * Chamar `RustFunctionName.startGame` para começar a emitir.
     */
    midiReadNote = "MIDI_READ_NOTE",
    /**
     * Evento para receber atualizacoes de estado da musica sendo tocada
     * 
     * O tipo retornado é `MidiState`.
     * 
     * Chamar `RustFunctionName.startGame` para começar a emitir.
     */
    midiReadState = "MIDI_READ_STATE",
}