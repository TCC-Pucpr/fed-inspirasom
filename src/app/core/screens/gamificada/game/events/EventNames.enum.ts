export enum EventNames {
    /** Emit when the [Game] scene is ready to start */
    gameSceneReady = "gameSceneReady",
    /** Emit when the [Pause] scene is ready to start */
    pauseSceneReady = "pauseSceneReady",
    /** Emit when the [End] scene is ready to start */
    endSceneReady = "endSceneReady",
    /** Emit when user is exiting the gamified mode */
    exitGame= "exitGame",
    /** Emit when user is pausing the gamified mode */
    pauseGame = "pauseGame",
    /** Emit when user is resuming the gamified mode */
    resumeGame = "resumeGame",
    /** Emit when the ocarina plays a note */
    ocarinaNote = "ocarinaNote",
    /** Emit when music state changes (playing, finished, paused) */
    musicStateChange = "stateChange",
    /** Emit when music ends */
    musicEnd = "musicEnd"
}