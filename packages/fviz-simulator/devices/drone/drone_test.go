package drone

import (
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestNew(t *testing.T) {
	t.Parallel()
	d := New("duid")
	assert.NotNil(t, d)
	assert.Equal(t, "duid", d.DUID)
}

func TestLoadData(t *testing.T) {
	t.Run("should error if it fails to open file", func(t *testing.T) {
		t.Parallel()
		d := New("duid")
		err := d.LoadData("fake")
		assert.Error(t, err)
	})

	t.Run("should load data", func(t *testing.T) {
		t.Parallel()
		d := New("duid")
		err := d.LoadData("data/test.fviz")
		require.NoError(t, err)
		assert.NotEmpty(t, d.flightData)
		assert.Greater(t, d.flightDataSize, 0)
	})
}

func TestStatus(t *testing.T) {
	t.Parallel()
	d := New("duid")
	_ = d.LoadData("data/test.fviz")
	prevPosition := d.currentPosition
	assert.NotEmpty(t, d.Status())
	assert.Greater(t, d.currentPosition, prevPosition)

	// Advance one line. Test file has only 2 lines
	_ = d.Status()
	assert.Equal(t, 0, d.currentPosition)
}
