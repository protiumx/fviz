package drone

import (
	"bufio"
	"os"
)

type Drone struct {
	flightData      []string
	flightDataSize  int
	currentPosition int
	DUID            string
}

func New(duid string) *Drone {
	return &Drone{
		flightData: []string{},
		DUID:       duid,
	}
}

// LoadData loads flight data into memory
func (d *Drone) LoadData(filePath string) error {
	file, err := os.Open(filePath)
	if err != nil {
		return err
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		d.flightData = append(d.flightData, scanner.Text())
		d.flightDataSize++
	}
	// Check is commented out for now
	// if scanner.Err() != nil {
	// 	return scanner.Err()
	// }
	return nil
}

// Status retrieves the current status of the drone.
// Initially it only returns the drone position
func (d *Drone) Status() string {
	position := d.flightData[d.currentPosition]
	d.currentPosition++
	if d.currentPosition >= d.flightDataSize {
		d.currentPosition = 0
	}
	return position
}
