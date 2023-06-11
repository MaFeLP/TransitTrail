/**
 * If the API should yield shorter, longer, or normal names.
 */
export enum Usage {
    /// No modification to the length of the outputs
    Normal = "Normal",

    /// Yields more verbose names
    Long = "Long",

    /// Yields terser names
    Short = "Short",
}
