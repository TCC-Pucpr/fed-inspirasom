/**
 * Todas as funcoes do rust
 *
 * chamadas via `invoke`
 *
 * Todas essas são thread safe e podem retornar algum tipo de erro.
 */
export enum RustFunctionName {
    // dataSource service
    /**
     * Conecta ao dispositivo midi com o nome esperado da ocarina
     */
    connectMidi = "start_listening_midi",
    /**
     * (port_name: String)
     *
     * Conecta a porta com o nome especificado. Idealmente ser um dos retornos de `listMidiDevices`
     */
    connectToMidiWithName = "connect_to_midi",
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
     * (music_id: number)
     *
     * Comeca a enviar o evento midiReadNote e midiReadState
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
    /**
     * (music_id: number)
     *
     * Calcula e devolve a duracao total da musica em segundos
     */
    musicLength = "music_length",
    /**
     * Retorna a quantidade de tempo (number) que ainda falta para terminar a musica em segundos.
     */
    remainingTime = "remaining_time",
    /**
     * (on_note_message: OnNotePressedMessage)
     *
     * Adiciona ao acumulador de score, retorna `OnScoreUpdateMessage`
     */
    onNote = "on_note",
    /**
     * (music_id: number)
     *
     * Reseta todos os scores de uma musica
     */
    resetMusicScore = "reset_music_score",
    /**
     * (music_id: number, order_type: ScoreOrderType, ascending: boolean | null, completed: boolean | null)
     *
     * Pega a lista de scores e tentativas feitas em uma musica, retornando um
     * lista de `Score`. ordenada baseado nos parametros.
     *
     * Se `ascending` for nulo, a lista é retornada na forma que foi armazenada.
     *
     * Se `completed` for nulo, retorna scores com a musica finalizada e sem ter finalizado
     *
     * Se for vazia, uma lista vazia é retornada.
     */
    listScores = "list_scores",
    /**
     * (music_name: String, file_path: String)
     * 
     * Cria uma nova musica no banco de dados
     * 
     * `file_path` é o caminho absoluto do arquivo, uma cópia desse arquivo é feito dentro do `resources`
     * se ele existir, e entao essa copia vai ser usada pelo app.
     * 
     * Retorna o novo `MidiMusic` adicionado
     */
    addNewMusic = "add_new_music",
    /**
     * (music_id: number)
     * 
     * Remove a musica e todos os seus scores da base.
     */
    removeMusic = "remove_music",
    
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