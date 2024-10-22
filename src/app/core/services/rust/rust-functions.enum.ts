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
     * Sinaliza que a musica foi concluida completamente e agora é necessário salvar
     * todas as informações coletadas durante a musica. Chamando essa funcao salva o score
     * com o estado final da musica como finalizada.
     * 
     * Essa função PRECISA ser chamada se quiser salvar todos os dados ao terminar a musica e também para 
     * resetar o estado da musica, se não nunca mais vai conseguir iniciar uma nova musica.
     * 
     * Voce so precisa chamar essa funcao se a musica for concluida, não chame essa função depois de 
     * chamar `stopGame`, se não vai retornar erro
     */
    endGame = "end_game",
    /**
     * Notifica o lado do rust para parar de enviar eventos de notas temporariamente
     */
    pauseGame = "pause_game",
    /**
     * Notifica o lado do rust para encerrar a emicao de eventos de notas
     */
    stopgame = "stop_game",
    /**
     * Notifica o lado do rust para voltar a emitir eventos de notas, só funciona se a música estiver
     * sido pausada.
     */
    resumeGame = "resume_game",
    /**
     * Devolve a lista de todas as musicas disponiveis (`MidiMusicList`)
     */
    listMusics = "list_musics",
    /**
     * (music_id: number)
     *
     * Calcula e devolve a duracao total da musica em segundos. Voce nao precisar
     * chamar essa função.
     */
    musicLength = "music_length",
    /**
     * Retorna a quantidade de tempo (number) que ainda falta para terminar a musica em segundos.
     */
    remainingTime = "remaining_time",
    /**
     * (on_note_message: number)
     *
     * Adiciona ao acumulador de score, retorna `OnScoreUpdateMessage`
     * 
     * Os numeros possíveis a ser usado sao:
     *
     * 0 - `Middle` para quando pressionar exatamente no momento certo
     * 1 - `Left` para quando estiver um pouco para esquerda
     * 2 - `Right` para quando estiver um pouco para direita
     * 3 - `Miss` quando deixar a nota passar
     * 4 - `EarlyMiss` quando errar a nota completamente antes de entrar na area de acerto
     */
    onNote = "on_note_played",
    /**
     * (music_id: number)
     *
     * Reseta todos os scores de uma musica
     */
    resetMusicScore = "reset_music_score",
    /**
     * (music_id: number, order_type: ScoreOrderType, ascending: boolean | null, completed: boolean | null)
     *
     * Pega a lista de scores e tentativas feitas em uma musica, retornando uma
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
     * Cria uma nova musica no banco de dados.
     *
     * `music_name` é apenas o nome da musica, não possui nenhuma relação com o nome do arquivo, pode ser
     * qualquer.
     * 
     * `file_path` é o caminho absoluto do arquivo, o arquivo é validado e uma cópia desse
     * arquivo é feito dentro do `resources`, essa copia é a que será usada quando começar a
     * jogar a musica.
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
    /**
     * Evento para receber sinais de conexao com o dispositivo MIDI.
     * 
     * Envia apenas um boolean, false quando for desconectado, true quando estiver conectado
     * 
     * Você vai receber esse evento quando conectar e desconectar, mas tambem pode receber `false` em 
     * qualquer momento enquanto tiver um dispositivo conectado, 
     * a verificação é feita a cada 3 segundos.
     */
    midiDeviceState = "MIDI_DEVICE_CONNECTION"
}