//! All the tooltip texts for buttons on the map are defined in here. 

use super::land::ButtonType;
use constants::*;
use definitions::TowerAttribute;

/// Chose language when creating. Lookup tooltips later using get_tooltip().
pub struct Tooltips {
	tips: Vec<String>,
}

impl Tooltips {
	pub fn get_tooltip(&self, b: &ButtonType) -> Option<String>{
		match *b {
			ButtonType::Concrete => Some(self.tips[0].clone()),
			ButtonType::Lumber => Some(self.tips[1].clone()),
			ButtonType::BuildIronFactory => Some(self.tips[2].clone()),
			ButtonType::UpgradeIronFactory{..} => Some(self.tips[3].clone()),
			ButtonType::BuildUniversity => Some(self.tips[4].clone()),
			ButtonType::UpgradeUniversity{..} => Some(self.tips[5].clone()),
			ButtonType::BuildBlacksmith => Some(self.tips[6].clone()),
			ButtonType::BuildOracle => Some(self.tips[7].clone()),
			ButtonType::BuildBank => Some(self.tips[8].clone()),
			ButtonType::UpgradeBank{..} => Some(self.tips[9].clone()),
			ButtonType::Industrialisation => Some(self.tips[10].clone()),
			ButtonType::EconomyResearch => Some(self.tips[11].clone()),
			ButtonType::ResearchTower{ index } if index == AOE_TID => Some(self.tips[12].clone()),
			ButtonType::ResearchTower{ index } if index == WALL_TID => Some(self.tips[13].clone()),
			ButtonType::ResearchTower{ index } if index == SLOW_TID => Some(self.tips[14].clone()),
			ButtonType::ResearchTower{ index } if index == ROCKET_TID => Some(self.tips[15].clone()),
			ButtonType::UpgradeGold{..} => Some(self.tips[16].clone()),
			ButtonType::UpgradeIron{..} => Some(self.tips[17].clone()),
			ButtonType::UpgradeCrystal{..} => Some(self.tips[18].clone()),
			ButtonType::UpgradeTower{ref kind, ..} => match *kind {
				TowerAttribute::Range => Some(self.tips[19].clone()),
				TowerAttribute::Attack => Some(self.tips[20].clone()),
				TowerAttribute::Defence => Some(self.tips[21].clone()),
			},
			ButtonType::BuildBlacksmithII => Some(self.tips[22].clone()),
			ButtonType::BuildBarracks => Some(self.tips[23].clone()),
			ButtonType::BuildArcheryRange => Some(self.tips[24].clone()),
			ButtonType::UpgradeCandy{..} => Some(self.tips[25].clone()),
			ButtonType::UpgradeSurprise{..} => Some(self.tips[26].clone()),
			_ => None
		}
	}
	pub fn new_en() -> Tooltips {
		let mut v = Vec::new();
		v.push(String::from("Concrete the land to build on it."));
		v.push(String::from("Chop down this tree to gain some wood."));
		v.push(String::from("Build a factory that produces iron over time."));
		v.push(String::from("Upgrade this iron factory. It will produce iron faster afterwards."));
		v.push(String::from("Build a university to unlock additional technologies."));
		v.push(String::from("Upgrade this university to unlock additional technologies."));
		v.push(String::from("Build a blacksmith to unlock and upgrade towers to defend your base."));
		v.push(String::from("Build a shrine to increase the income earned in mini games."));
		v.push(String::from("Build a bank that earns you gold coins over time."));
		v.push(String::from("Extend this bank so it will produce more gold coins."));
		v.push(String::from("Research industrialisation. This will give you access to new buildings."));
		v.push(String::from("Research economy. This will give you access to new buildings."));
		v.push(String::from("Unlock a tower that delights sad faces on a circular area arround itself."));
		v.push(String::from("Unlock walls and use them to redirect the incomming sad faces."));
		v.push(String::from("Unlock cotton candy that can slow down sad faces."));
		v.push(String::from("Invent chocolate to get access to a new awesome tower."));
		v.push(String::from("Gain more gold coins by winning mini games."));
		v.push(String::from("Gain more iron by winning mini games."));
		v.push(String::from("Gain more crystals by winning mini games."));
		v.push(String::from("Increase the range of the tower on the picture."));
		v.push(String::from("Increase the delighting power of the tower on the picture."));
		v.push(String::from("Make the tower on the picture stronger so it does not fall too easily."));
		v.push(String::from("Extend the blacksmith so it can increase the delighting power of some towers."));
		v.push(String::from("Extend the blacksmith so it can increase the sturdiness of some towers."));
		v.push(String::from("Extend the blacksmith so it can increase the range of some towers."));
		v.push(String::from("Upgrade the cotton candy tower so it can slow more sad faces."));
		v.push(String::from("Upgrade the chocolate tower so it can delight more sad faces."));
		Tooltips { tips: v }
	}
	pub fn new_de() -> Tooltips {
		let mut v = Vec::new();
		v.push(String::from("Betoniere das Grundstück um darauf zu bauen."));
		v.push(String::from("Fälle diesen Baum um Holz zu gewinnen."));
		v.push(String::from("Baue eine Eisenfabrik."));
		v.push(String::from("Baue diese Eisenfabrik aus damit sie schneller Eisen produziert."));
		v.push(String::from("Errichte eine Universität um weitere Technologien freizuschalten."));
		v.push(String::from("Baue diese Universität weiter aus um weitere Technologien freizuschalten."));
		v.push(String::from("Baue eine Schmiede um Verteidigungsanlagen zu erforschen und zu verbessern."));
		v.push(String::from("Baue einen Schrein um bei den Minispielen mehr Ressourcen zu erhalten."));
		v.push(String::from("Baue eine Bank, die dir regelmässig Goldmünzen einbringt."));
		v.push(String::from("Vergrössere diese Bank um mehr Goldmünzen zu erhalten."));
		v.push(String::from("Erforsche die Industrialisierung. Dadurch werden neue Gebäude freigeschalten"));
		v.push(String::from("Erforsche die Wirtschaft. Dadurch werden neue Gebäude freigeschalten"));
		v.push(String::from("Schalte einen Turm frei der alle nahen Schmollmünder gleichzeitig aufheitert."));
		v.push(String::from("Schalte Zäune frei und verwende sie um die Schmollmünder umzulenken."));
		v.push(String::from("Schalte Zuckerwatte frei die dir hilft Schmollmünder zu verlangsamen."));
		v.push(String::from("Erfinde Schokolade um Zugriff zu einem tollen, neuen Turm zu erhalten."));
		v.push(String::from("Erhalte mehr Gold durch Siege in Minispielen."));
		v.push(String::from("Erhalte mehr Eisen durch Siege in Minispielen."));
		v.push(String::from("Erhalte mehr Kristalle durch Siege in Minispielen."));
		v.push(String::from("Erhöhe die Reichweite des Turmes auf dem Bild."));
		v.push(String::from("Erhöhe die Aufmunterungskraft des Turmes auf dem Bild."));
		v.push(String::from("Verstärke den Turm auf dem Bild damit er schwieriger kaputt geht."));
		v.push(String::from("Baue die Schmiede aus damit sie die Aufmunterungskraft einiger Türme verbessern kann."));
		v.push(String::from("Baue die Schmiede aus damit sie die Robustheit einiger Türme verbessern kann."));
		v.push(String::from("Baue die Schmiede aus damit sie die Reichweite einiger Türme verbessern kann."));
		v.push(String::from("Entwickle den Zuckerwattenstand weiter, damit mehr Schmollmünder verlangsamt werden können."));
		v.push(String::from("Entwickle den Schokoladenturm weiter, damit mehr Schmollmünder aufgeheitert werden können."));
		Tooltips { tips: v }
	}
}