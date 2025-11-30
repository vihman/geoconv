use proj::{Coord, Proj};

#[derive(Debug)]
pub struct Transform {
    proj: Proj
}

impl Transform {
    pub fn new(from: &str, to: &str)->Self {
        let proj = match Proj::new_known_crs(from, to, None) {
            Ok(proj) => proj,
            Err(e) => panic!("{e}")
        };
        
        Self {
            proj: proj
        }
    }


    pub fn convert(&self, p: (f64, f64)) -> Result<(f64, f64), proj::ProjError> {
        self.proj.convert(p)
    }
}

//Convert from WGS84 to L-EST97
pub fn wgs84_to_est97(x: &f64, y: &f64) -> Result<(f64, f64), proj::ProjError> {
    Transform::new("epsg:4326", "epsg:3301").convert((*y, *x))
} 

// Convert from WSG84 to L-EST97
pub fn est97_to_wgs84(x: &f64, y: &f64) -> Result<(f64, f64), proj::ProjError> {
    Transform::new("epsg:3301", "epsg:4326").convert((*y, *x))
}


pub struct CoordStruct {
    x: f64,
    y: f64
}

impl Coord<f64> for CoordStruct {
    
    fn x(&self) -> f64 {
        self.x
    }
    
    fn y(&self) -> f64 {
        self.y
    }
    
    fn from_xy(x: f64, y: f64 ) -> Self {
        Self {
            x: x,
            y: y
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_est97_to_wgs84() {
        let result = est97_to_wgs84(&6585021.15, &549443.23 );
        assert_eq!(result.1, 59.400235813566866);
        assert_eq!(result.0, 24.870385830398387);
    }

        #[test]
    fn test_wgs84_to_est97() {
        let result = wgs84_to_est97(&59.400235813566866, &24.870385830398387);
        assert_eq!(result.1, 6585021.15);
        assert_eq!(result.0, 549443.23);
    }

    #[test]
    #[should_panic]
    fn test_wrong_identifier() {
        Transform::new("EPSG4326", "cabbage");
    }
}