pub async fn exists_by_name(state: &AppState, email: &str) -> Result<bool, String> {
    let vivienda = Viviendas::find()
        .filter(viviendas::Column::name.contains(email))
        .one(&state.conn)
        .await;

    match vivienda {
        Ok(vivienda) => Ok(vivienda.is_some()),
        Err(e) => Err(e.to_string()),
    }
}
