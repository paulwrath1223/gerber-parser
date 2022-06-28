use gerber_parser::parser::parse_gerber;
mod utils;

/// All files in these tests are taken from the Ucamco 20220409 file format examples
/// downloaded from https://www.ucamco.com/en/gerber/downloads on 20220628

#[test]
fn two_square_boxes_to_rust_and_back() {
    let gbr_string = "
    %FSLAX26Y26*%
    %MOMM*%
    %TF.Part,Other,example*%
    %LPD*%
    %ADD10C,0.010*%
    D10*
    X0Y0D02*
    G01*
    X5000000Y0D01*
    Y5000000D01*
    X0D01*
    Y0D01*
    X6000000D02*
    X11000000D01*
    Y5000000D01*
    X6000000D01*
    Y0D01*
    M02*        
    ";
    let reader = utils::gerber_to_reader(&gbr_string);

    assert_eq!(gbr_string, utils::gerber_doc_to_str(parse_gerber(reader)))
}

#[test]
fn polarities_and_apertures_to_rust_and_back() {
    let gbr_string = "
    %FSLAX36Y36*%
    %MOMM*%
    %TF.FileFunction,Other,Sample*%
    %LPD*%
    G04 Define Apertures*
    %AMTHERMAL80*
    7,0,0,0.800,0.550,0.125,45*%
    %ADD10C,0.1*%
    %ADD11C,0.6*%
    %ADD12R,0.6X0.6*%
    %ADD13R,0.4X1.00*%
    %ADD14R,1.00X0.4*%
    %ADD15O,0.4X01.00*%
    %ADD16P,1.00X3*%
    %ADD19THERMAL80*%
    G04 Start image generation*
    D10*
    X0Y2500000D02*
    G01*
    X0Y0D01*
    X2500000Y0D01*
    X10000000Y10000000D02*
    X15000000D01*
    X20000000Y15000000D01*
    X25000000D02*
    Y10000000D01*
    D11*
    X10000000Y10000000D03*
    X20000000D03*
    X25000000D03*
    Y15000000D03*
    X20000000D03*
    D12*
    X10000000Y15000000D03*
    D13*
    X30000000Y15000000D03*
    D14*
    Y12500000D03*
    D15*
    Y10000000D03*
    D10*
    X37500000Y10000000D02*
    G75*
    G03*
    X37500000Y10000000I2500000J0D01*
    D16*
    X34000000Y10000000D03*
    X35000000Y9000000D03*
    G36*
    X5000000Y20000000D02*
    G01*
    Y37500000D01*
    X37500000D01*
    Y20000000D01*
    X5000000D01*
    G37*
    %LPC*%
    G36*
    X10000000Y25000000D02*
    Y30000000D01*
    G02*
    X12500000Y32500000I2500000J0D01*
    G01*
    X30000000D01*
    G02*
    X30000000Y25000000I0J-3750000D01*
    G01*
    X10000000D01*
    G37*
    %LPD*%
    D10*
    X15000000Y28750000D02*
    X20000000D01*
    D11*
    X15000000Y28750000D03*
    X20000000D03*
    D19*
    X28750000Y28750000D03*
    M02*      
    ";
    let reader = utils::gerber_to_reader(&gbr_string);

    assert_eq!(gbr_string, utils::gerber_doc_to_str(parse_gerber(reader)))
}

#[test]
fn a_drill_file_to_rust_and_back() {
    let gbr_string = "
    %FSLAX36Y36*%
    %MOMM*%
    %TF.FileFunction,Plated,1,8,PTH*%
    %TF.Part,Single*%
    %LPD*%
    %TA.DrillTolerance,0.002,0.001*%
    %TA.AperFunction,ComponentDrill*%
    %ADD10C,0.14000*%
    %TA.AperFunction,Other,SpecialDrill*%
    %ADD11C,0.24000*%
    %TA.DrillTolerance,0.15,0.15*%
    %TA.AperFunction,MechanicalDrill*%
    %ADD12C,0.43000*%
    %ADD13C,0.22000*%
    %TD.AperFunction*%
    %TD.DrillTolerance*%
    G01*
    D10*
    X2420000Y2750000D03*
    Y3250000D03*
    X2170000Y3000000D03*
    X1920000Y3250000D03*
    X2920000Y2750000D03*
    X1920000D03*
    X2920000Y3250000D03*
    X2670000Y3000000D03*
    D11*
    X1240000Y0D03*
    X0Y-1240000D03*
    X-1240000Y0D03*
    X880000Y880000D03*
    X-880000D03*
    X0Y1240000D03*
    X880000Y-880000D03*
    X-880000D03*
    D12*
    X7920000Y3500000D03*
    X4920000Y-3500000D03*
    D13*
    X7670000Y-6000000D03*
    X5670000D03*
    X-2330000Y2000000D03*
    Y4000000D03*
    Y0D03*
    Y-2000000D03*
    Y-6000000D03*
    Y-4000000D03*
    X-330000Y-6000000D03*
    X1670000D03*
    X3670000D03*
    M02*     
    ";
    let reader = utils::gerber_to_reader(&gbr_string);

    assert_eq!(gbr_string, utils::gerber_doc_to_str(parse_gerber(reader)))
}

