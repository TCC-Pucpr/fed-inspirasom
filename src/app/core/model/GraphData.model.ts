export interface GraphData {
    /**
     * Data na qual as sessões foram realizadas
     * 
     * 
     * Como não há previsão de localização do inspirasom, a data será tratada como dd/mm/yy
     */
    date: string,

    /**
     * Valor numérico que metrifica a performance do usuario durante a sessão
     */
    score: number
}