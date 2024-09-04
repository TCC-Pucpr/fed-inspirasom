// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { OnNotePrecision } from "./NotePressPrecision";

/**
 * Struct para quando a nota for pressionada pelo usuario.
 *
 * Poderá ter 4 estados:
 * `Middle` para quando pressionar exatamente no momento certo
 * `Left` para quando estiver um pouco para esquerda
 * `Right` para quando estiver um pouco para direita
 * `Miss` quando deixar a nota passar
 * `EarlyMiss` quando errar a nota completamente antes de entrar na area de acerto
 */
export type OnNoteMessage = { precision: OnNotePrecision, };
