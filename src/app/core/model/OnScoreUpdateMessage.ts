// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.

/**
 * Payload que será emitido ao front sobre sempre que o usuario atualizar o seu score
 * o `total_score` é o score total acumulado na sessao atual da musica e o
 * `latest_message_score` é o score ganho/perdido depois do ultimo input.
 */
export type OnScoreUpdateMessage = { hit_streak: number, total_score: bigint, latest_message_score: number, };
