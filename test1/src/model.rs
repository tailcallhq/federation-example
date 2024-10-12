use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ABigObject {
	a_field_on_a_big_object: String,
	b_field_on_a_big_object: i32,
	c_field_on_a_big_object: bool,
	d_field_on_a_big_object: f64,
	e_field_on_a_big_object: String,
	f_field_on_a_big_object: i32,
	g_field_on_a_big_object: bool,
	h_field_on_a_big_object: f64,
	i_field_on_a_big_object: String,
	j_field_on_a_big_object: i32,
	k_field_on_a_big_object: bool,
	l_field_on_a_big_object: f64,
	m_field_on_a_big_object: String,
	n_field_on_a_big_object: i32,
	o_field_on_a_big_object: bool,
	p_field_on_a_big_object: f64,
	q_field_on_a_big_object: String,
	r_field_on_a_big_object: i32,
	s_field_on_a_big_object: bool,
	t_field_on_a_big_object: f64,
	u_field_on_a_big_object: String,
	v_field_on_a_big_object: i32,
	w_field_on_a_big_object: bool,
	x_field_on_a_big_object: f64,
	y_field_on_a_big_object: String,
	z_field_on_a_big_object: i32,
}


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BBigObject {
	a_field_on_b_big_object: String,
	b_field_on_b_big_object: i32,
	c_field_on_b_big_object: bool,
	d_field_on_b_big_object: f64,
	e_field_on_b_big_object: String,
	f_field_on_b_big_object: i32,
	g_field_on_b_big_object: bool,
	h_field_on_b_big_object: f64,
	i_field_on_b_big_object: String,
	j_field_on_b_big_object: i32,
	k_field_on_b_big_object: bool,
	l_field_on_b_big_object: f64,
	m_field_on_b_big_object: String,
	n_field_on_b_big_object: i32,
	o_field_on_b_big_object: bool,
	p_field_on_b_big_object: f64,
	q_field_on_b_big_object: String,
	r_field_on_b_big_object: i32,
	s_field_on_b_big_object: bool,
	t_field_on_b_big_object: f64,
	u_field_on_b_big_object: String,
	v_field_on_b_big_object: i32,
	w_field_on_b_big_object: bool,
	x_field_on_b_big_object: f64,
	y_field_on_b_big_object: String,
	z_field_on_b_big_object: i32,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CBigObject {
    a_field_on_c_big_object: String,
    b_field_on_c_big_object: i32,
    c_field_on_c_big_object: bool,
    d_field_on_c_big_object: f64,
    e_field_on_c_big_object: String,
    f_field_on_c_big_object: i32,
    g_field_on_c_big_object: bool,
    h_field_on_c_big_object: f64,
    i_field_on_c_big_object: String,
    j_field_on_c_big_object: i32,
    k_field_on_c_big_object: bool,
    l_field_on_c_big_object: f64,
    m_field_on_c_big_object: String,
    n_field_on_c_big_object: i32,
    o_field_on_c_big_object: bool,
    p_field_on_c_big_object: f64,
    q_field_on_c_big_object: String,
    r_field_on_c_big_object: i32,
    s_field_on_c_big_object: bool,
    t_field_on_c_big_object: f64,
    u_field_on_c_big_object: String,
    v_field_on_c_big_object: i32,
    w_field_on_c_big_object: bool,
    x_field_on_c_big_object: f64,
    y_field_on_c_big_object: String,
    z_field_on_c_big_object: i32,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DBigObject {
    a_field_on_d_big_object: String,
    b_field_on_d_big_object: i32,
    c_field_on_d_big_object: bool,
    d_field_on_d_big_object: f64,
    e_field_on_d_big_object: String,
    f_field_on_d_big_object: i32,
    g_field_on_d_big_object: bool,
    h_field_on_d_big_object: f64,
    i_field_on_d_big_object: String,
    j_field_on_d_big_object: i32,
    k_field_on_d_big_object: bool,
    l_field_on_d_big_object: f64,
    m_field_on_d_big_object: String,
    n_field_on_d_big_object: i32,
    o_field_on_d_big_object: bool,
    p_field_on_d_big_object: f64,
    q_field_on_d_big_object: String,
    r_field_on_d_big_object: i32,
    s_field_on_d_big_object: bool,
    t_field_on_d_big_object: f64,
    u_field_on_d_big_object: String,
    v_field_on_d_big_object: i32,
    w_field_on_d_big_object: bool,
    x_field_on_d_big_object: f64,
    y_field_on_d_big_object: String,
    z_field_on_d_big_object: i32,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FBigObject {
    a_field_on_f_big_object: String,
    b_field_on_f_big_object: i32,
    c_field_on_f_big_object: bool,
    d_field_on_f_big_object: f64,
    e_field_on_f_big_object: String,
    f_field_on_f_big_object: i32,
    g_field_on_f_big_object: bool,
    h_field_on_f_big_object: f64,
    i_field_on_f_big_object: String,
    j_field_on_f_big_object: i32,
    k_field_on_f_big_object: bool,
    l_field_on_f_big_object: f64,
    m_field_on_f_big_object: String,
    n_field_on_f_big_object: i32,
    o_field_on_f_big_object: bool,
    p_field_on_f_big_object: f64,
    q_field_on_f_big_object: String,
    r_field_on_f_big_object: i32,
    s_field_on_f_big_object: bool,
    t_field_on_f_big_object: f64,
    u_field_on_f_big_object: String,
    v_field_on_f_big_object: i32,
    w_field_on_f_big_object: bool,
    x_field_on_f_big_object: f64,
    y_field_on_f_big_object: String,
    z_field_on_f_big_object: i32,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GBigObject {
    a_field_on_g_big_object: String,
    b_field_on_g_big_object: i32,
    c_field_on_g_big_object: bool,
    d_field_on_g_big_object: f64,
    e_field_on_g_big_object: String,
    f_field_on_g_big_object: i32,
    g_field_on_g_big_object: bool,
    h_field_on_g_big_object: f64,
    i_field_on_g_big_object: String,
    j_field_on_g_big_object: i32,
    k_field_on_g_big_object: bool,
    l_field_on_g_big_object: f64,
    m_field_on_g_big_object: String,
    n_field_on_g_big_object: i32,
    o_field_on_g_big_object: bool,
    p_field_on_g_big_object: f64,
    q_field_on_g_big_object: String,
    r_field_on_g_big_object: i32,
    s_field_on_g_big_object: bool,
    t_field_on_g_big_object: f64,
    u_field_on_g_big_object: String,
    v_field_on_g_big_object: i32,
    w_field_on_g_big_object: bool,
    x_field_on_g_big_object: f64,
    y_field_on_g_big_object: String,
    z_field_on_g_big_object: i32,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LBigObject {
    a_field_on_l_big_object: String,
    b_field_on_l_big_object: i32,
    c_field_on_l_big_object: bool,
    d_field_on_l_big_object: f64,
    e_field_on_l_big_object: String,
    f_field_on_l_big_object: i32,
    g_field_on_l_big_object: bool,
    h_field_on_l_big_object: f64,
    i_field_on_l_big_object: String,
    j_field_on_l_big_object: i32,
    k_field_on_l_big_object: bool,
    l_field_on_l_big_object: f64,
    m_field_on_l_big_object: String,
    n_field_on_l_big_object: i32,
    o_field_on_l_big_object: bool,
    p_field_on_l_big_object: f64,
    q_field_on_l_big_object: String,
    r_field_on_l_big_object: i32,
    s_field_on_l_big_object: bool,
    t_field_on_l_big_object: f64,
    u_field_on_l_big_object: String,
    v_field_on_l_big_object: i32,
    w_field_on_l_big_object: bool,
    x_field_on_l_big_object: f64,
    y_field_on_l_big_object: String,
    z_field_on_l_big_object: i32,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MBigObject {
    a_field_on_m_big_object: String,
    b_field_on_m_big_object: i32,
    c_field_on_m_big_object: bool,
    d_field_on_m_big_object: f64,
    e_field_on_m_big_object: String,
    f_field_on_m_big_object: i32,
    g_field_on_m_big_object: bool,
    h_field_on_m_big_object: f64,
    i_field_on_m_big_object: String,
    j_field_on_m_big_object: i32,
    k_field_on_m_big_object: bool,
    l_field_on_m_big_object: f64,
    m_field_on_m_big_object: String,
    n_field_on_m_big_object: i32,
    o_field_on_m_big_object: bool,
    p_field_on_m_big_object: f64,
    q_field_on_m_big_object: String,
    r_field_on_m_big_object: i32,
    s_field_on_m_big_object: bool,
    t_field_on_m_big_object: f64,
    u_field_on_m_big_object: String,
    v_field_on_m_big_object: i32,
    w_field_on_m_big_object: bool,
    x_field_on_m_big_object: f64,
    y_field_on_m_big_object: String,
    z_field_on_m_big_object: i32,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PBigObject {
    a_field_on_p_big_object: String,
    b_field_on_p_big_object: i32,
    c_field_on_p_big_object: bool,
    d_field_on_p_big_object: f64,
    e_field_on_p_big_object: String,
    f_field_on_p_big_object: i32,
    g_field_on_p_big_object: bool,
    h_field_on_p_big_object: f64,
    i_field_on_p_big_object: String,
    j_field_on_p_big_object: i32,
    k_field_on_p_big_object: bool,
    l_field_on_p_big_object: f64,
    m_field_on_p_big_object: String,
    n_field_on_p_big_object: i32,
    o_field_on_p_big_object: bool,
    p_field_on_p_big_object: f64,
    q_field_on_p_big_object: String,
    r_field_on_p_big_object: i32,
    s_field_on_p_big_object: bool,
    t_field_on_p_big_object: f64,
    u_field_on_p_big_object: String,
    v_field_on_p_big_object: i32,
    w_field_on_p_big_object: bool,
    x_field_on_p_big_object: f64,
    y_field_on_p_big_object: String,
    z_field_on_p_big_object: i32,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct QBigObject {
    a_field_on_q_big_object: String,
    b_field_on_q_big_object: i32,
    c_field_on_q_big_object: bool,
    d_field_on_q_big_object: f64,
    e_field_on_q_big_object: String,
    f_field_on_q_big_object: i32,
    g_field_on_q_big_object: bool,
    h_field_on_q_big_object: f64,
    i_field_on_q_big_object: String,
    j_field_on_q_big_object: i32,
    k_field_on_q_big_object: bool,
    l_field_on_q_big_object: f64,
    m_field_on_q_big_object: String,
    n_field_on_q_big_object: i32,
    o_field_on_q_big_object: bool,
    p_field_on_q_big_object: f64,
    q_field_on_q_big_object: String,
    r_field_on_q_big_object: i32,
    s_field_on_q_big_object: bool,
    t_field_on_q_big_object: f64,
    u_field_on_q_big_object: String,
    v_field_on_q_big_object: i32,
    w_field_on_q_big_object: bool,
    x_field_on_q_big_object: f64,
    y_field_on_q_big_object: String,
    z_field_on_q_big_object: i32,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RBigObject {
    a_field_on_r_big_object: String,
    b_field_on_r_big_object: i32,
    c_field_on_r_big_object: bool,
    d_field_on_r_big_object: f64,
    e_field_on_r_big_object: String,
    f_field_on_r_big_object: i32,
    g_field_on_r_big_object: bool,
    h_field_on_r_big_object: f64,
    i_field_on_r_big_object: String,
    j_field_on_r_big_object: i32,
    k_field_on_r_big_object: bool,
    l_field_on_r_big_object: f64,
    m_field_on_r_big_object: String,
    n_field_on_r_big_object: i32,
    o_field_on_r_big_object: bool,
    p_field_on_r_big_object: f64,
    q_field_on_r_big_object: String,
    r_field_on_r_big_object: i32,
    s_field_on_r_big_object: bool,
    t_field_on_r_big_object: f64,
    u_field_on_r_big_object: String,
    v_field_on_r_big_object: i32,
    w_field_on_r_big_object: bool,
    x_field_on_r_big_object: f64,
    y_field_on_r_big_object: String,
    z_field_on_r_big_object: i32,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SBigObject {
    a_field_on_s_big_object: String,
    b_field_on_s_big_object: i32,
    c_field_on_s_big_object: bool,
    d_field_on_s_big_object: f64,
    e_field_on_s_big_object: String,
    f_field_on_s_big_object: i32,
    g_field_on_s_big_object: bool,
    h_field_on_s_big_object: f64,
    i_field_on_s_big_object: String,
    j_field_on_s_big_object: i32,
    k_field_on_s_big_object: bool,
    l_field_on_s_big_object: f64,
    m_field_on_s_big_object: String,
    n_field_on_s_big_object: i32,
    o_field_on_s_big_object: bool,
    p_field_on_s_big_object: f64,
    q_field_on_s_big_object: String,
    r_field_on_s_big_object: i32,
    s_field_on_s_big_object: bool,
    t_field_on_s_big_object: f64,
    u_field_on_s_big_object: String,
    v_field_on_s_big_object: i32,
    w_field_on_s_big_object: bool,
    x_field_on_s_big_object: f64,
    y_field_on_s_big_object: String,
    z_field_on_s_big_object: i32,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TBigObject {
    a_field_on_t_big_object: String,
    b_field_on_t_big_object: i32,
    c_field_on_t_big_object: bool,
    d_field_on_t_big_object: f64,
    e_field_on_t_big_object: String,
    f_field_on_t_big_object: i32,
    g_field_on_t_big_object: bool,
    h_field_on_t_big_object: f64,
    i_field_on_t_big_object: String,
    j_field_on_t_big_object: i32,
    k_field_on_t_big_object: bool,
    l_field_on_t_big_object: f64,
    m_field_on_t_big_object: String,
    n_field_on_t_big_object: i32,
    o_field_on_t_big_object: bool,
    p_field_on_t_big_object: f64,
    q_field_on_t_big_object: String,
    r_field_on_t_big_object: i32,
    s_field_on_t_big_object: bool,
    t_field_on_t_big_object: f64,
    u_field_on_t_big_object: String,
    v_field_on_t_big_object: i32,
    w_field_on_t_big_object: bool,
    x_field_on_t_big_object: f64,
    y_field_on_t_big_object: String,
    z_field_on_t_big_object: i32,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UBigObject {
    a_field_on_u_big_object: String,
    b_field_on_u_big_object: i32,
    c_field_on_u_big_object: bool,
    d_field_on_u_big_object: f64,
    e_field_on_u_big_object: String,
    f_field_on_u_big_object: i32,
    g_field_on_u_big_object: bool,
    h_field_on_u_big_object: f64,
    i_field_on_u_big_object: String,
    j_field_on_u_big_object: i32,
    k_field_on_u_big_object: bool,
    l_field_on_u_big_object: f64,
    m_field_on_u_big_object: String,
    n_field_on_u_big_object: i32,
    o_field_on_u_big_object: bool,
    p_field_on_u_big_object: f64,
    q_field_on_u_big_object: String,
    r_field_on_u_big_object: i32,
    s_field_on_u_big_object: bool,
    t_field_on_u_big_object: f64,
    u_field_on_u_big_object: String,
    v_field_on_u_big_object: i32,
    w_field_on_u_big_object: bool,
    x_field_on_u_big_object: f64,
    y_field_on_u_big_object: String,
    z_field_on_u_big_object: i32,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct XBigObject {
    a_field_on_x_big_object: String,
    b_field_on_x_big_object: i32,
    c_field_on_x_big_object: bool,
    d_field_on_x_big_object: f64,
    e_field_on_x_big_object: String,
    f_field_on_x_big_object: i32,
    g_field_on_x_big_object: bool,
    h_field_on_x_big_object: f64,
    i_field_on_x_big_object: String,
    j_field_on_x_big_object: i32,
    k_field_on_x_big_object: bool,
    l_field_on_x_big_object: f64,
    m_field_on_x_big_object: String,
    n_field_on_x_big_object: i32,
    o_field_on_x_big_object: bool,
    p_field_on_x_big_object: f64,
    q_field_on_x_big_object: String,
    r_field_on_x_big_object: i32,
    s_field_on_x_big_object: bool,
    t_field_on_x_big_object: f64,
    u_field_on_x_big_object: String,
    v_field_on_x_big_object: i32,
    w_field_on_x_big_object: bool,
    x_field_on_x_big_object: f64,
    y_field_on_x_big_object: String,
    z_field_on_x_big_object: i32,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct YBigObject {
    a_field_on_y_big_object: String,
    b_field_on_y_big_object: i32,
    c_field_on_y_big_object: bool,
    d_field_on_y_big_object: f64,
    e_field_on_y_big_object: String,
    f_field_on_y_big_object: i32,
    g_field_on_y_big_object: bool,
    h_field_on_y_big_object: f64,
    i_field_on_y_big_object: String,
    j_field_on_y_big_object: i32,
    k_field_on_y_big_object: bool,
    l_field_on_y_big_object: f64,
    m_field_on_y_big_object: String,
    n_field_on_y_big_object: i32,
    o_field_on_y_big_object: bool,
    p_field_on_y_big_object: f64,
    q_field_on_y_big_object: String,
    r_field_on_y_big_object: i32,
    s_field_on_y_big_object: bool,
    t_field_on_y_big_object: f64,
    u_field_on_y_big_object: String,
    v_field_on_y_big_object: i32,
    w_field_on_y_big_object: bool,
    x_field_on_y_big_object: f64,
    y_field_on_y_big_object: String,
    z_field_on_y_big_object: i32,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ZBigObject {
    a_field_on_z_big_object: String,
    b_field_on_z_big_object: i32,
    c_field_on_z_big_object: bool,
    d_field_on_z_big_object: f64,
    e_field_on_z_big_object: String,
    f_field_on_z_big_object: i32,
    g_field_on_z_big_object: bool,
    h_field_on_z_big_object: f64,
    i_field_on_z_big_object: String,
    j_field_on_z_big_object: i32,
    k_field_on_z_big_object: bool,
    l_field_on_z_big_object: f64,
    m_field_on_z_big_object: String,
    n_field_on_z_big_object: i32,
    o_field_on_z_big_object: bool,
    p_field_on_z_big_object: f64,
    q_field_on_z_big_object: String,
    r_field_on_z_big_object: i32,
    s_field_on_z_big_object: bool,
    t_field_on_z_big_object: f64,
    u_field_on_z_big_object: String,
    v_field_on_z_big_object: i32,
    w_field_on_z_big_object: bool,
    x_field_on_z_big_object: f64,
    y_field_on_z_big_object: String,
    z_field_on_z_big_object: i32,
}

// type EnumType string

// const (
// 	EnumTypeA EnumType = "A"
// 	EnumTypeB EnumType = "B"
// 	EnumTypeC EnumType = "C"
// )

// var AllEnumType = []EnumType{
// 	EnumTypeA,
// 	EnumTypeB,
// 	EnumTypeC,
// }

// func (e EnumType) IsValid() bool {
// 	switch e {
// 	case EnumTypeA, EnumTypeB, EnumTypeC:
// 		return true
// 	}
// 	return false
// }

// func (e EnumType) String() string {
// 	return string(e)
// }

// func (e *EnumType) UnmarshalGQL(v interface{}) error {
// 	str, ok := v.(string)
// 	if !ok {
// 		return fmt.Errorf("enums must be strings")
// 	}

// 	*e = EnumType(str)
// 	if !e.IsValid() {
// 		return fmt.Errorf("%s is not a valid EnumType", str)
// 	}
// 	return nil
// }

// func (e EnumType) MarshalGQL(w io.Writer) {
// 	fmt.Fprint(w, strconv.Quote(e.String()))
// }


impl Default for ABigObject {
    fn default() -> Self {
        ABigObject {
            a_field_on_a_big_object: "a field on a big object - lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur excepteur sint occaecat cupidatat non proident sunt in culpa qui officia deserunt mollit anim id est laborum".to_string(),
            b_field_on_a_big_object: 1,
            c_field_on_a_big_object: true,
            d_field_on_a_big_object: 2.0,
            e_field_on_a_big_object: "e field on a big object - lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur excepteur sint occaecat cupidatat non proident sunt in culpa qui officia deserunt mollit anim id est laborum".to_string(),
            f_field_on_a_big_object: 3,
            g_field_on_a_big_object: true,
            h_field_on_a_big_object: 4.0,
            i_field_on_a_big_object: "i field on a big object - lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur excepteur sint occaecat cupidatat non proident sunt in culpa qui officia deserunt mollit anim id est laborum".to_string(),
            j_field_on_a_big_object: 5,
            k_field_on_a_big_object: true,
            l_field_on_a_big_object: 6.0,
            m_field_on_a_big_object: "m field on a big object - lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur excepteur sint occaecat cupidatat non proident sunt in culpa qui officia deserunt mollit anim id est laborum".to_string(),
            n_field_on_a_big_object: 7,
            o_field_on_a_big_object: true,
            p_field_on_a_big_object: 8.0,
            q_field_on_a_big_object: "q field on a big object - lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur excepteur sint occaecat cupidatat non proident sunt in culpa qui officia deserunt mollit anim id est laborum".to_string(),
            r_field_on_a_big_object: 9,
            s_field_on_a_big_object: true,
            t_field_on_a_big_object: 10.0,
            u_field_on_a_big_object: "u field on a big object - lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur excepteur sint occaecat cupidatat non proident sunt in culpa qui officia deserunt mollit anim id est laborum".to_string(),
            v_field_on_a_big_object: 11,
            w_field_on_a_big_object: true,
            x_field_on_a_big_object: 12.0,
            y_field_on_a_big_object: "y field on a big object - lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur excepteur sint occaecat cupidatat non proident sunt in culpa qui officia deserunt mollit anim id est laborum".to_string(),
            z_field_on_a_big_object: 13,
        }
    }
}


#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DeeplyNestedObject {
    a_field_on_deeply_nested_object: String,
    b_field_on_deeply_nested_object: i32,
    c_field_on_deeply_nested_object: bool,
    d_field_on_deeply_nested_object: f64,
    e_field_on_deeply_nested_object: String,
    f_field_on_deeply_nested_object: i32,
    g_field_on_deeply_nested_object: bool,
    h_field_on_deeply_nested_object: f64,
    i_field_on_deeply_nested_object: String,
    j_field_on_deeply_nested_object: i32,
    k_field_on_deeply_nested_object: bool,
    l_field_on_deeply_nested_object: f64,
    m_field_on_deeply_nested_object: String,
    n_field_on_deeply_nested_object: i32,
    o_field_on_deeply_nested_object: bool,
    p_field_on_deeply_nested_object: f64,
    q_field_on_deeply_nested_object: String,
    r_field_on_deeply_nested_object: i32,
    s_field_on_deeply_nested_object: bool,
    t_field_on_deeply_nested_object: f64,
    u_field_on_deeply_nested_object: String,
    v_field_on_deeply_nested_object: i32,
    w_field_on_deeply_nested_object: bool,
    x_field_on_deeply_nested_object: f64,
    y_field_on_deeply_nested_object: String,
    z_field_on_deeply_nested_object: i32,
}

impl Default for DeeplyNestedObject {
    fn default() -> Self {
        DeeplyNestedObject {
            a_field_on_deeply_nested_object: "a field on deeply nested object - Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.".to_string(),
            b_field_on_deeply_nested_object: 1,
            c_field_on_deeply_nested_object: true,
            d_field_on_deeply_nested_object: 2.0,
            e_field_on_deeply_nested_object: "e field on deeply nested object - Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.".to_string(),
            f_field_on_deeply_nested_object: 3,
            g_field_on_deeply_nested_object: false,
            h_field_on_deeply_nested_object: 4.0,
            i_field_on_deeply_nested_object: "i field on deeply nested object - Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.".to_string(),
            j_field_on_deeply_nested_object: 5,
            k_field_on_deeply_nested_object: true,
            l_field_on_deeply_nested_object: 6.0,
            m_field_on_deeply_nested_object: "m field on deeply nested object - Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.".to_string(),
            n_field_on_deeply_nested_object: 7,
            o_field_on_deeply_nested_object: false,
            p_field_on_deeply_nested_object: 8.0,
            q_field_on_deeply_nested_object: "q field on deeply nested object - Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.".to_string(),
            r_field_on_deeply_nested_object: 9,
            s_field_on_deeply_nested_object: true,
            t_field_on_deeply_nested_object: 10.0,
            u_field_on_deeply_nested_object: "u field on deeply nested object - Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.".to_string(),
            v_field_on_deeply_nested_object: 11,
            w_field_on_deeply_nested_object: false,
            x_field_on_deeply_nested_object: 12.0,
            y_field_on_deeply_nested_object: "y field on deeply nested object - Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.".to_string(),
            z_field_on_deeply_nested_object: 13,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NestedObject {
    pub deeply_nested_objects: Vec<DeeplyNestedObject>,
}

impl Default for NestedObject {
    fn default() -> Self {
        NestedObject {
            deeply_nested_objects: vec![DeeplyNestedObject::default()],
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BigObject {
    pub nested_objects: Vec<NestedObject>,
}

impl Default for BigObject {
    fn default() -> Self {
        BigObject {
            nested_objects: vec![NestedObject::default()],
        }
    }
}